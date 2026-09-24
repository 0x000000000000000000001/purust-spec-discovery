module Test.Main2 where

import Prelude

import Effect (Effect)
import Test.Spec.Discovery (discoverAndRunSpecs)
import Test.Spec.Reporter.Console (consoleReporter)

-- | A pattern that matches no module must run an empty suite and exit 0.
main :: Effect Unit
main = discoverAndRunSpecs [consoleReporter] "Does\\.Not\\.Match"
