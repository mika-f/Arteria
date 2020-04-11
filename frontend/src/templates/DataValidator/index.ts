// Perl5 Data::Validator Example
import { DirectoryItem, FileItem, Item } from "@mika-f/monaka";

import { Dependency } from "../../models/dependency";

export const guid = "5eaa7024-3ff8-43c9-8ac0-69353249be31";
export const title = "Data::Validator";
export const dependencies: Dependency[] = [{ nameWithVersion: "Data::Validator" }];
export const command = "perl main.pl";
export const project: Item[] = [
  {
    type: "directory",
    id: "a957aebf-9764-4d8b-9f13-6346c29ba506",
    title: "lib",
    parentId: null,
    state: "opened"
  } as DirectoryItem,
  {
    type: "file",
    id: "f426051c-e9ec-4c9e-928c-e7ec8e7b23b6",
    title: "Validator.pm",
    content: `
package Validator;
use strict;
use warnings;
use utf8;
use features qw/say/;

use Data::Validator;

sub validate {
  state $v; $v //= Data::Validator->new(
    str => 'Str',
  )->with(qw/Method/);
  my ($self, $args) = $v->validate(@_);
  my ($str,)        = $args->{str};

  say "Hello, $str!";
}

1;
`.trim(),
    parentId: "a957aebf-9764-4d8b-9f13-6346c29ba506"
  } as FileItem,
  {
    type: "file",
    id: "43a7a4b2-1981-4b1b-86e7-22d8b7439d63",
    title: "main.pl",
    content: `
package main;
use strict;
use warnings;
use utf8;

use Validator;

Validator->validate(str => 'World');

1;
`.trim(),
    parentId: null
  } as FileItem
];
