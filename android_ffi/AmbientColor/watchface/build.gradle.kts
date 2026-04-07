plugins {
    alias(libs.plugins.android.application)
}

android {
    namespace = "it.mconst.ambientcolor"
    compileSdk {
        version = release(36) {
            minorApiLevel = 1
        }
    }

    defaultConfig {
        applicationId = "it.mconst.ambientcolor"
        minSdk = 36
        targetSdk = 36
        versionCode = 1
        versionName = "1.0"

    }

    buildTypes {
        release {
            isMinifyEnabled = false
            proguardFiles(
                getDefaultProguardFile("proguard-android-optimize.txt"),
                "proguard-rules.pro"
            )
        }
    }
    enableKotlin = false
}

dependencies {
}