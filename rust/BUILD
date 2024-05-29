package(default_visibility = ["//visibility:public"])

load("@rules_rust//proto/protobuf:toolchain.bzl", "GRPC_COMPILE_DEPS")
#load("@crate_index//:defs.bzl", "all_crate_deps")
load("@rules_rust//rust:defs.bzl", "rust_binary")

rust_binary(
    name = "cli",
    srcs = ["cli/main.rs"],
    deps =  GRPC_COMPILE_DEPS + ["//proto:helloworld_grpc"],
)

rust_binary(
    name = "srv",
    srcs = ["srv/main.rs"],
    deps = GRPC_COMPILE_DEPS + ["//proto:helloworld_grpc"],
)
