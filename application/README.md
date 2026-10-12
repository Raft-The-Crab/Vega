# Application

This directory contains all application code for VEGA.

## Structure

- **desktop/** - C#/.NET 11 + Avalonia desktop application (Windows/macOS)
- **android/** - Kotlin + Jetpack Compose Android application

## Desktop Application

See `desktop/README.md` for details on the desktop application structure.

## Android Application

See `android/README.md` for details on the Android application structure.

## Shared Code

Application-specific shared code should be placed in platform-appropriate subdirectories.
Cross-platform shared logic lives in `../core/`.
