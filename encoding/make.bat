@echo off

set build_mode=%1

echo %build_mode%

IF %build_mode%==--release (
    set RUSTFLAGS=-Clink-args=/DEBUG:NONE
    cargo build --release
) ELSE (
    cargo build
)