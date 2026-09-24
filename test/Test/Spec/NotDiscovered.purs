module Test.Spec.NotDiscovered where

import Prelude

import Test.Spec (Spec, describe, it)
import Test.Spec.Assertions (shouldEqual)

-- | This module exports a `spec`, but its name does not match the pattern the
-- | test suite discovers with, so this failing test must never run.
spec :: Spec Unit
spec =
  describe "not discovered" do
    it "must never run" do
      1 `shouldEqual` 2
