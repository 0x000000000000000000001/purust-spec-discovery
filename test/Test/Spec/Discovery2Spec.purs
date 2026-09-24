module Test.Spec.Discovery2Spec where

import Prelude

import Test.Spec (Spec, describe, it)
import Test.Spec.Assertions (shouldEqual)

spec :: Spec Unit
spec =
  describe "discovery" do
    it "discovers the specs again in this project" do
      (2 + 2) `shouldEqual` 4
