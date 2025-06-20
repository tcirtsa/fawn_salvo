// Jenkinsfile
pipeline {
    agent {
        // 使用一个包含 Rust 工具链的 Docker 镜像作为构建环境
        // 这里可以使用官方的 rust 镜像，或者你自己构建一个包含特定依赖的镜像
        docker {
            image 'rust:latest' // 使用最新的 Rust 官方镜像
            args '-u root' // 有时在容器内构建需要root权限，但如果可能应避免
        }
    }
    stages {
        stage('Checkout') {
            steps {
                // Jenkins会自动将代码检出到工作区，但显式写出更清晰
                checkout scm
            }
        }
        stage('Build Rust Project') {
            steps {
                script {
                    // 进入项目目录
                    dir('fawn_salvo') { // 根据你的仓库结构，可能不需要这行，如果repo根目录就是项目
                        // 构建 Rust 项目
                        sh 'cargo build --release'
                    }
                }
            }
        }
        stage('Run Tests (Optional)') {
            steps {
                script {
                    dir('fawn_salvo') {
                        // 运行 Rust 测试
                        sh 'cargo test'
                    }
                }
            }
        }
        stage('Package/Deploy (Optional)') {
            steps {
                script {
                    dir('fawn_salvo') {
                        // 示例：将构建好的二进制文件打包或复制到其他位置
                        // 例如，如果构建产物在 target/release/fawn_salvo
                        sh 'ls -lh target/release/'
                        // 可以使用 Docker 构建最终的应用镜像
                        // build an image from a Dockerfile in your repo
                        // sh 'docker build -t my-fawn-salvo-app .'
                        // 或者复制二进制文件
                        // sh 'cp target/release/fawn_salvo /path/to/deploy'
                    }
                }
            }
        }
    }
    post {
        always {
            // 构建完成后清理工作区
            cleanWs()
        }
        success {
            echo 'Build successful!'
        }
        failure {
            echo 'Build failed!'
        }
    }
}