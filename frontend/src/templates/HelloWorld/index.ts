// Perl5 Hello, World Example
import { FileItem, Item } from "@mika-f/monaka";

import { Dependency } from "../../models/dependency";

export const guid = "c5137ac8-9c5d-45d9-aa1b-a50caa84210d";
export const title = "Hello, World";
export const dependencies: Dependency[] = [];
export const command = "perl main.pl";
export const project: Item[] = [
  {
    type: "file",
    id: "d2bf7b7d-1ac5-c250-6112-ab75b6c08371",
    title: "main.pl",
    content: `
use strict;
use warnings;
use utf8;
use feature qw/say/;

say "Hello, World";

1;
`.trim(),
    parentId: null
  } as FileItem
];
