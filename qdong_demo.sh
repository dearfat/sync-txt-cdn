#!/bin/bash

# 函数：显示帮助信息
show_help() {
    echo "Usage: $0 [-mode=MODE] [-port=PORT]"
    echo
    echo "Options:"
    echo "  -mode=MODE    Set the mode (e.g., dev, prod)"
    echo "  -port=PORT    Set the port number"
    echo "  -h, --help    Show this help message"
}

# 默认参数值
MODE="default"
PORT="80"

# 解析命令行参数
for i in "$@"; do
    case $i in
        -mode=*)
            MODE="${i#*=}"
            shift
            ;;
        -port=*)
            PORT="${i#*=}"
            shift
            ;;
        -h|--help)
            show_help
            exit 0
            ;;
        *)
            echo "Unknown option: $i"
            show_help
            exit 1
            ;;
    esac
done

# 检测操作系统平台
OS="$(uname -s)"
case "$OS" in
    Linux*)     PLATFORM="Linux";;
    Darwin*)    PLATFORM="macOS";;
    CYGWIN*)    PLATFORM="Cygwin";;
    MINGW*)     PLATFORM="MinGw";;
    *)          PLATFORM="Unknown";;
esac

# 输出结果
echo "Operating System: $PLATFORM"
echo "Mode: $MODE"
echo "Port: $PORT"

# 根据平台执行不同的操作
if [ "$PLATFORM" = "Linux" ]; then
    echo "Running on Linux"
    # 在这里添加 Linux 特定的操作
elif [ "$PLATFORM" = "macOS" ]; then
    echo "Running on macOS"
    # 在这里添加 macOS 特定的操作
elif [ "$PLATFORM" = "Cygwin" ] || [ "$PLATFORM" = "MinGw" ]; then
    echo "Running on Windows"
    # 在这里添加 Windows 特定的操作
else
    echo "Unknown operating system"
    # 在这里添加其他操作系统的操作
fi