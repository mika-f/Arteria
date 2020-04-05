import React from "react";

import { Button, PrimaryButton, LinkButton } from ".";

export default {
  title: "atoms/Buttons"
};

export const Default = () => <Button>Normal Button</Button>;

export const Primary = () => <PrimaryButton>Primary Button</PrimaryButton>;

export const Link = () => <LinkButton>Link Button</LinkButton>;
