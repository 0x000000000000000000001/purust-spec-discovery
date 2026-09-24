module Test.Spec.DiscoverySpec where

import Prelude

import Test.Spec (Spec, describe, it)
import Test.Spec.Assertions (shouldEqual)

spec :: Spec Unit
spec =
  describe "discovery" do
    it "discovers the specs in this project" do
      (1 + 1) `shouldEqual` 2
