# slip39-rust

![Rust compilation results](https://github.com/Internet-of-People/slip39-rust/workflows/Rust/badge.svg)

[SLIP-0039](https://github.com/satoshilabs/slips/blob/master/slip-0039.md) compatible secret sharing tool

## Table of Contents <!-- omit in toc -->

- [Installation](#installation)
- [Usage](#usage)
  - [Getting help](#getting-help)
  - [Generate master secret and split it to parts](#generate-master-secret-and-split-it-to-parts)
  - [Split an existing secret (hex or BIP-0039)](#split-an-existing-secret-hex-or-bip-0039)
  - [Inspect a member share](#inspect-a-member-share)
- [Contributing](#contributing)

## Installation

For now we do not make a binary release, but after installing rust and checking out the repository you can create
the binary for your platform. On a Unix, these steps might give you and idea:

```sh
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
...
$ rustc --version
rustc 1.37.0 (eae3437df 2019-08-13)
$ cargo --version
cargo 1.37.0 (9edd08916 2019-08-02)
$ git clone https://github.com/Internet-of-People/slip39-rust.git
...
$ cd slip39-rust
$ cargo install --path . --force
$ slip39 --version
slip39 0.1.0
```

## Usage

### Getting help

Most of this README.md is compiled from the actual help available in the tool.

```sh
$ slip39 --help
...
$ slip39 split --help
...
```

### Generate master secret and split it to parts

SLIP-0039 defines a 2-level split: The master secret is split into group secrets and then those are split further into
member secrets. You can define the required and total number of members in each group, and also define how many groups
are required to restore the master secret.

```sh
# The example from the specification
$ slip39 generate --password "morpheus" --bits 256 --required-groups 2 --group 1of1 --group 1of1 --group 3of5 --group 2of6
{
  "group_count": 4,
  "group_threshold": 2,
  "groups": [
    {
      "member_threshold": 1,
      "member_count": 1,
      "shares": [
        {
          "group_index": 1,
          "member_index": 1,
          "mnemonic": "transfer flea acrobat romp anatomy leader axis impulse premium junction salt type smith maximum class clogs ruler talent alpha exchange alien total debut early presence skunk mixed platform dramatic provide center pumps year"
        }
      ]
    },
    {
      "member_threshold": 1,
      "member_count": 1,
      "shares": [
        {
          "group_index": 2,
          "member_index": 1,
          "mnemonic": "transfer flea beard romp always loyalty grasp adequate wildlife petition identify duke fake umbrella explain various uncover diploma wits volume sprinkle enjoy seafood prevent welcome voting elevator flame coastal charity detailed timely antenna"
        }
      ]
    },
    {
      "member_threshold": 3,
      "member_count": 5,
      "shares": [
        {
          "group_index": 3,
          "member_index": 1,
          "mnemonic": "transfer flea ceramic round ajar magazine clogs ending listen cover flip sack anxiety hunting shaft fatal again alien union express vexed grin database smoking rhyme carve again valid idle smoking toxic clock nervous"
        },
        {
          "group_index": 3,
          "member_index": 2,
          "mnemonic": "transfer flea ceramic scatter aunt mortgage fancy admit clothes slavery rebuild isolate dough scout explain usual evoke filter tracks strategy kitchen wits slavery fatal elite grant spray regret iris device season intend blessing"
        },
        {
          "group_index": 3,
          "member_index": 3,
          "mnemonic": "transfer flea ceramic shaft avoid pajamas literary budget duckling recover living critical axle graduate scramble glimpse afraid glimpse orange seafood subject fridge frequent quantity require merit umbrella guest trial starting email amuse flip"
        },
        {
          "group_index": 3,
          "member_index": 4,
          "mnemonic": "transfer flea ceramic skin agree revenue vanish funding orbit frequent have mixed category slim elegant ruin evening debris move eyebrow fancy segment rhythm debut enemy true drift ceramic unwrap demand grasp forget spirit"
        },
        {
          "group_index": 3,
          "member_index": 5,
          "mnemonic": "transfer flea ceramic snake ajar blanket froth promise mountain public news infant toxic broken purchase velvet idea educate mineral alive ecology umbrella expand wrist erode infant mule makeup rumor veteran faint spark literary"
        }
      ]
    },
    {
      "member_threshold": 2,
      "member_count": 6,
      "shares": [
        {
          "group_index": 4,
          "member_index": 1,
          "mnemonic": "transfer flea decision roster alive frequent flash enjoy flash arena hazard disease walnut overall finger paper papa silent software capture company royal trend necklace romp sympathy trash merit surface exotic analysis tadpole curious"
        },
        {
          "group_index": 4,
          "member_index": 2,
          "mnemonic": "transfer flea decision scared auction pitch mandate ivory trip episode speak activity crisis slavery prize species listen grasp believe webcam racism sheriff beaver category drove bracelet answer easy season fantasy remember stick client"
        },
        {
          "group_index": 4,
          "member_index": 3,
          "mnemonic": "transfer flea decision shadow agency depict victim drove material enjoy acne evaluate frozen dismiss regret eclipse fluff soul example spirit public space adjust lily critical maiden detect friar very ranked overall salon amuse"
        },
        {
          "group_index": 4,
          "member_index": 4,
          "mnemonic": "transfer flea decision sister argue season counter adequate debris adjust reject improve marvel hawk element demand knit laden mental deny cinema surface western dream sprinkle elite sprinkle march unkind pipeline daisy science acne"
        },
        {
          "group_index": 4,
          "member_index": 5,
          "mnemonic": "transfer flea decision smug actress scholar angel realize elbow formal reunion rebound agency mustang mortgage august easy distance upstairs marathon remove thumb clay skunk alive ranked epidemic amuse nylon duckling empty length guilt"
        },
        {
          "group_index": 4,
          "member_index": 6,
          "mnemonic": "transfer flea decision spew ambition decorate squeeze magazine zero cargo airline medal standard scatter hunting kidney golden multiple depend hearing credit unfair swimming inform welfare lawsuit plot stilt losing civil view living genre"
        }
      ]
    }
  ]
}
```

### Split an existing secret (hex or [BIP-0039](https://github.com/bitcoin/bips/blob/master/bip-0039.mediawiki))

```sh
# Nothing prevents you to use a single member share if you like that better :)
$ slip39 split --password "morpheus" --entropy "shell view flock obvious believe final afraid caught page second arrow predict" --group 1of1
{
  "group_count": 1,
  "group_threshold": 1,
  "groups": [
    {
      "member_threshold": 1,
      "member_count": 1,
      "shares": [
        {
          "group_index": 1,
          "member_index": 1,
          "mnemonic": "location recover academic academic easel false playoff galaxy process strategy exercise forecast yoga union execute herd problem luck dynamic already"
        }
      ]
    }
  ]
}
```

### Inspect a member share

```sh
$ slip39 inspect --mnemonic "location recover academic academic easel false playoff galaxy process strategy exercise forecast yoga union execute herd problem luck dynamic already"
{
  "identifier": 17239,
  "iterations": 0,
  "group_threshold": 1,
  "group_index": 1,
  "member_threshold": 1,
  "member_index": 1
}
```

## Contributing

Feel free to open issues and send pull requests in this repository. By sending contributions, you are agreeing to transfer
all intellectual property from your changes to the Decentralized Society Foundation, Panama, who owns the copyright of this
code.
