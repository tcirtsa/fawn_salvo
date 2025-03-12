use crate::connect;
use crate::db::schema::users::dsl::*;
use crate::model::user_model::User;
use base64::{engine::general_purpose::STANDARD, Engine as _};
use diesel::prelude::*;
use rodio::{buffer::SamplesBuffer, Source};
use salvo::{prelude::*, Error};
use serde::{Deserialize, Serialize};
use spectrum_analyzer::{samples_fft_to_spectrum, FrequencyLimit};
use std::io::Cursor;

#[derive(Deserialize)]
struct VoiceData {
    username: String,
    voice_data: String, // Base64编码的声纹数据
}

#[derive(Deserialize)]
struct TextToSpeechRequest {
    username: String,
    #[allow(dead_code)]
    text: String,
}

#[derive(Serialize)]
struct AudioResponse {
    audio_data: String, // Base64编码的音频数据
}

#[handler]
pub async fn collect_voice_fingerprint(req: &mut Request, res: &mut Response) -> Result<(), Error> {
    let mut conn = connect().unwrap();
    let voice_data = req.parse_json::<VoiceData>().await?;

    // 解码Base64音频数据
    let audio_bytes = STANDARD
        .decode(&voice_data.voice_data)
        .map_err(|e| Error::other(format!("Failed to decode base64: {}", e)))?;

    // 将音频数据转换为samples
    let cursor = Cursor::new(audio_bytes);
    let decoder = rodio::Decoder::new(cursor)
        .map_err(|e| Error::other(format!("Failed to decode audio: {}", e)))?;

    // 先获取采样率
    let sample_rate = decoder.sample_rate();

    // 获取音频样本并转换类型
    let samples: Vec<f32> = decoder.map(|s| s as f32 / i16::MAX as f32).collect();

    // 使用FFT提取频谱特征
    let spectrum_config = FrequencyLimit::Range(20.0, 20000.0);
    let spectrum = samples_fft_to_spectrum(&samples, sample_rate, spectrum_config, None)
        .map_err(|e| Error::other(format!("Failed to analyze spectrum: {:?}", e)))?;

    // 将频谱数据序列化为字节
    let voice_features: Vec<u8> = spectrum
        .data()
        .iter()
        .flat_map(|(freq, mag)| {
            let mut bytes = Vec::new();
            let freq_f32: f32 = freq.val();
            let mag_f32: f32 = mag.val();
            bytes.extend_from_slice(&freq_f32.to_le_bytes());
            bytes.extend_from_slice(&mag_f32.to_le_bytes());
            bytes
        })
        .collect();

    // 将特征数据转换为base64
    let voice_features_base64 = STANDARD.encode(&voice_features);

    // 更新用户的声纹数据
    let result = diesel::update(users.filter(username.eq(&voice_data.username)))
        .set(voice_fingerprint.eq(&voice_features_base64))
        .execute(&mut conn);

    match result {
        Ok(_) => {
            res.render(Text::Plain("声纹采集成功"));
            Ok(())
        }
        Err(e) => {
            res.render(Json(&e.to_string()));
            Ok(())
        }
    }
}

#[handler]
pub async fn text_to_speech(req: &mut Request, res: &mut Response) -> Result<(), Error> {
    let mut conn = connect().unwrap();
    let tts_request = req.parse_json::<TextToSpeechRequest>().await?;

    // 获取用户的声纹数据
    let user = users
        .filter(username.eq(&tts_request.username))
        .first::<User>(&mut conn)
        .optional()
        .map_err(|e| Error::other(format!("Database error: {}", e)))?;

    match user {
        Some(user) => {
            if let Some(_voice_fingerprint) = user.voice_fingerprint {
                // 生成简单的正弦波作为基础音频
                let sample_rate = 44100;
                let duration = 2.0; // 2秒
                let frequency = 440.0; // A4音符

                let samples: Vec<f32> = (0..(sample_rate as f32 * duration) as usize)
                    .map(|i| {
                        let t = i as f32 / sample_rate as f32;
                        (2.0 * std::f32::consts::PI * frequency * t).sin()
                    })
                    .collect();

                // 创建音频buffer
                let buffer = SamplesBuffer::new(1, sample_rate, samples);

                // 将音频数据转换为bytes
                let audio_data: Vec<u8> = buffer
                    .convert_samples::<i16>()
                    .map(|sample| sample.to_le_bytes().to_vec())
                    .flatten()
                    .collect();

                // 将音频数据转换为base64格式
                let audio_base64 = STANDARD.encode(&audio_data);

                let audio_response = AudioResponse {
                    audio_data: audio_base64,
                };
                res.render(Json(audio_response));
            } else {
                res.render(Text::Plain("用户未采集声纹数据"));
            }
            Ok(())
        }
        None => {
            res.render(Text::Plain("用户不存在"));
            Ok(())
        }
    }
}
