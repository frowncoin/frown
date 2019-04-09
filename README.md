[![Build Status](https://img.shields.io/travis/mimblewimble/frown/master.svg)](https://travis-ci.org/mimblewimble/frown)
[![Coverage Status](https://img.shields.io/codecov/c/github/mimblewimble/frown/master.svg)](https://codecov.io/gh/mimblewimble/frown)
[![Chat](https://img.shields.io/gitter/room/frown_community/Lobby.svg)](https://gitter.im/frown_community/Lobby)
[![Support](https://img.shields.io/badge/support-on%20gitter-brightgreen.svg)](https://gitter.im/frown_community/support)
[![Documentation Wiki](https://img.shields.io/badge/doc-wiki-blue.svg)](https://github.com/mimblewimble/docs/wiki)
[![Release Version](https://img.shields.io/github/release/mimblewimble/frown.svg)](https://github.com/mimblewimble/frown/releases)
[![License](https://img.shields.io/github/license/mimblewimble/frown.svg)](https://github.com/mimblewimble/frown/blob/master/LICENSE)

# Frown

Frown is an in-progress implementation of the MimbleWimble protocol. Many characteristics are still undefined but the following constitutes a first set of choices:

  * Clean and minimal implementation, and aiming to stay as such.
  * Follows the MimbleWimble protocol, which provides great anonymity and scaling characteristics.
  * Cuckoo Cycle proof of work in two variants named Cuckaroo (ASIC-resistant) and Cuckatoo (ASIC-targeted).
  * Relatively fast block time: one minute.
  * Fixed block reward over time with a decreasing dilution.
  * Transaction fees are based on the number of Outputs created/destroyed and total transaction size.
  * Smooth curve for difficulty adjustments.

To learn more, read our [introduction to MimbleWimble and Frown](doc/intro.md).

## Status

Frown is live with mainnet. Still, much is left to be done and [contributions](CONTRIBUTING.md) are welcome (see below). Check our [mailing list archives](https://lists.launchpad.net/mimblewimble/) for the latest status.

## Contributing

To get involved, read our [contributing docs](CONTRIBUTING.md).

Find us:

* Chat: [Gitter](https://gitter.im/frown_community/Lobby).
* Mailing list: join the [~MimbleWimble team](https://launchpad.net/~mimblewimble) and subscribe on Launchpad.
* Twitter for the Frown council: [@frowncouncil](https://twitter.com/frowncouncil)

## Getting Started

To learn more about the technology, read our [introduction](doc/intro.md).

To build and try out Frown, see the [build docs](doc/build.md).

## Philosophy

Frown likes itself small and easy on the eyes. It wants to be inclusive and welcoming for all walks of life, without judgement. Frown is terribly ambitious, but not at the detriment of others, rather to further us all. It may have strong opinions to stay in line with its objectives, which doesn't mean disrespect of others' ideas.

We believe in pull requests, data and scientific research. We do not believe in unfounded beliefs.

## Credits

Tom Elvis Jedusor for the first formulation of MimbleWimble.

Andrew Poelstra for his related work and improvements.

John Tromp for the Cuckoo Cycle proof of work.

J.K. Rowling for making it despite extraordinary adversity.

## License

Apache License v2.0.
