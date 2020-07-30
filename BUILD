proto_library(
    name = "node_proto",
    srcs = [
        "src/node.proto",
    ],
)

cc_proto_library(
    name = "node_cc_proto",
    deps = [
        ":node_proto",
    ],
)

cc_test(
    name = "test",
    srcs = ["test.cc"],
    deps = [
        ":node_cc_proto",
        "@com_google_protobuf//:protobuf_lite",
        "@gtest",
        "@gtest//:gtest_main",
    ],
)
