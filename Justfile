set dotenv-path := "private/.env"

set shell := ["nu", "-c"]

default:
    just --list

# Build for Windows
windows:
    mkdir dist
    dx build --platform windows -r
    cp --recursive ./target/dx/smarter-buaa/release/windows/app ./dist/SmarterBuaa-x86_64-windows-msvc --force
alias w := windows

# Build for Android
android:  _android_remove_private_config _android_add_private_config && _android_remove_private_config
    mkdir dist
    dx build --platform android --arch arm64 -r
    cp ./target/dx/smarter-buaa/release/android/app/app/build/outputs/apk/release/app-release.apk ./dist/SmarterBuaa-aarch64-linux-android.apk --force
alias a := android

# Android Build Auxiliary Recipe
_jks_encoder:
    open --raw private/fontlos.jks | encode base64 | save ./private/fontlos.txt
_jks_decoder:
    $env.JKS_BASE64 | decode base64 | save ./private/fontlos.jks
_android_add_private_config:
    #! nu
    if 'ANDROID_CONFIG_BLOCK' in $env {
        let dx_config = (open --raw './Dioxus.toml' | str trim --right)
        let target = (
            "\n[bundle.android]\n" +
            $"jks_file = \"($env.JKS_FILE)\"\n" +
            $"jks_password = \"($env.JKS_PASSWORD)\"\n" +
            $"key_alias = \"($env.KEY_ALIAS)\"\n" +
            $"key_password = \"($env.KEY_PASSWORD)\""
        )
        let new_config = $"($dx_config)\n($target)\n"
        $new_config | save './Dioxus.toml' --force
    } else {
        return
    }
_android_remove_private_config:
    #! nu
    if 'ANDROID_CONFIG_BLOCK' in $env {
        let dx_config = (open --raw './Dioxus.toml' | str trim --right)
        let target = (
            "\n[bundle.android]\n" +
            $"jks_file = \"($env.JKS_FILE)\"\n" +
            $"jks_password = \"($env.JKS_PASSWORD)\"\n" +
            $"key_alias = \"($env.KEY_ALIAS)\"\n" +
            $"key_password = \"($env.KEY_PASSWORD)\""
        )
        let new_config = ($dx_config | str replace $target '')
        $new_config | save './Dioxus.toml' --force
    } else {
        return
    }
