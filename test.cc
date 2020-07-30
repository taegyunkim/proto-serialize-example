#include "gmock/gmock.h"
#include "gtest/gtest.h"

#include "src/node.pb.h"

TEST(SerializeTest, FromUnsignedChar) {
  unsigned char data[5] = {10, 3, 97, 98, 99};

  Node node;
  ASSERT_TRUE(node.ParseFromArray(&data, 5));

  EXPECT_EQ(node.id(), "abc");
}