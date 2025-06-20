pipeline {
    agent any // 或者你为流水线指定的具体代理

    stages {
        stage('Checkout') {
            steps {
                echo '开始检出代码...'
                // ***** 这一行是关键！ *****
                // 它会使用你在 Jenkins UI 中配置的 GitHub URL、分支、凭据等信息，
                // 实际地将你的项目代码克隆到 /var/jenkins_home/workspace/nbplus 目录。
                checkout scm
            }
        }

        stage('Build with Docker') {
            steps {
                script {
                    docker.image('rust:1.78').inside {
                        echo '验证 Rust 和 Cargo 版本...'
                        sh 'rustc --version'
                        sh 'cargo --version'

                        echo '开始构建 Rust 项目...'
                        // 因为 `checkout scm` 已经把代码拉下来了，
                        // 现在 `Cargo.toml` 就应该存在于 `/var/jenkins_home/workspace/nbplus` 目录中了。
                        sh 'cargo build --release'
                    }
                }
            }
        }

        // ... 其他你需要的阶段
    }

    // 后置操作，例如通知、清理等
    post {
        always {
            echo '流水线执行完毕。'
        }
        success {
            echo '流水线成功完成！'
        }
        failure {
            echo '流水线失败，请检查日志寻找错误原因。'
        }
    }
}