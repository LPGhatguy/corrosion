<h1 align="center">Stormtide: Corrosion</h1>
<div align="center">
	<a href="https://travis-ci.org/StormtideGame/stormtide-core">
		<img src="https://travis-ci.org/StormtideGame/corrosion.svg?branch=master" alt="Travis CI Build Status" />
	</a>
</div>

<div align="center">
	<strong><i>Magic</i> rules engine implemented in Rust</strong>
</div>

<div>&nbsp;</div>

This is an attempt to implement a rules implementation for *Magic* in Rust.

This repository just contains the core rules engine and a small program to bootstrap tests. A UI for actually playing the game is out of scope and will be present in a separate repository.

## Goals
* Clean, readability-focused codebase
* High rules accuracy and coverage
* Deterministic game replays
* High performance, but zero crashes

## Future Plans
* Actually have a reasonable subset of the game
* Compile to WASM, build a browser UI

## License
Corrosion is available under the MIT license. See [LICENSE.md](LICENSE.md) for details.