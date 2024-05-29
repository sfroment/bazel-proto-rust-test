load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

# To find additional information on this release or newer ones visit:
# https://github.com/bazelbuild/rules_rust/releases
http_archive(
    name = "rules_rust",
    integrity = "sha256-+bWb47wg0VchIADaHt6L5Dma2Gn+Q589nz/MKcTi+lo=",
    urls = ["https://github.com/bazelbuild/rules_rust/releases/download/0.45.1/rules_rust-v0.45.1.tar.gz"],
)

load("@rules_rust//rust:repositories.bzl", "rules_rust_dependencies", "rust_register_toolchains")

rules_rust_dependencies()

rust_register_toolchains(edition = "2021")

#load("@rules_rust//crate_universe:repositories.bzl", "crate_universe_dependencies")

#crate_universe_dependencies(bootstrap = True)

# protobuf support

load("@rules_rust//proto/protobuf:repositories.bzl", "rust_proto_protobuf_dependencies", "rust_proto_protobuf_register_toolchains")

rust_proto_protobuf_dependencies()

rust_proto_protobuf_register_toolchains()

#load("@com_google_protobuf//:protobuf_deps.bzl", "protobuf_deps")
#load("@rules_proto//proto:repositories.bzl", "rules_proto_dependencies")

#rules_proto_dependencies()

#protobuf_deps()

load("@rules_rust//proto/protobuf:transitive_repositories.bzl", "rust_proto_protobuf_transitive_repositories")

rust_proto_protobuf_transitive_repositories()

#load("@rules_rust//proto/protobuf:transitive_repositories.bzl", "rust_proto_protobuf_transitive_repositories")

#rust_proto_protobuf_transitive_repositories()

# Cargo support

#load("@rules_rust//crate_universe:defs.bzl", "crates_repository")

#crates_repository(
#    name = "crate_index",
#    cargo_lockfile = "//rust:Cargo.lock",
#    manifests = ["//rust:Cargo.toml"],
#)

#load("@crate_index//:defs.bzl", "crate_repositories")

#crate_repositories()

# rust-analyzer
load("@rules_rust//tools/rust_analyzer:deps.bzl", "rust_analyzer_dependencies")

rust_analyzer_dependencies()

