import React from "react";

import { ListContainer, ListItem } from ".";

export default {
  title: "atoms/List"
};

export const Default = () => (
  <ListContainer>
    <ListItem>Item 1</ListItem>
    <ListItem>Item 2</ListItem>
  </ListContainer>
);
