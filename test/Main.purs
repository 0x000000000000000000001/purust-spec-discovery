module Test.Main where

import Prelude

import Effect (Effect)
import Effect.Aff (launchAff_)
import Effect.Class (liftEffect)
import Test.Spec.Discovery2Spec as Discovery2Spec
import Test.Spec.DiscoverySpec as DiscoverySpec
import Test.Spec.Reporter.Console (consoleReporter)
import Test.Spec.Runner.Node (runSpecAndExitProcess)

-- Native binaries cannot discover compiled modules dynamically, so the two
-- discovery specs are listed explicitly. `Test.Spec.Discovery.discover` fails
-- with an explicit message on the native backend.
main :: Effect Unit
main = launchAff_ $
  liftEffect $ runSpecAndExitProcess [ consoleReporter ]
    (DiscoverySpec.spec *> Discovery2Spec.spec)
