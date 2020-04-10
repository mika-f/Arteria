// Perl5 Data::Dumper Example
import { FileItem, Item } from "@mika-f/monaka";

import { Dependency } from "../../models/dependency";

export const guid = "f47c23be-17a7-45c9-85b3-df62b4133df1";
export const title = "Data::Dumper";
export const dependencies: Dependency[] = [];
export const command = "perl main.pl";
export const project: Item[] = [
  {
    type: "file",
    id: "d2bf7b7d-1ac5-c250-6112-ab75b6c08371",
    title: "main.pl",
    content: `
package main;
use Data::Dumper;
    
print Dumper { a => 1 };
`.trim(),
    parentId: null
  } as FileItem
];
