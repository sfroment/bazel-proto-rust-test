load("@rules_proto//proto:defs.bzl", "proto_library")
load("@rules_rust//proto/protobuf:defs.bzl", "rust_grpc_library")

package(default_visibility = ["//visibility:public"])

proto_library(
    name = "helloworld",
    srcs = ["helloworld.proto"]
)

rust_grpc_library(
    name = "helloworld_grpc",
    deps = [":helloworld"],
)
