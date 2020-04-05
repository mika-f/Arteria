import React from "react";

import FontAwesome from ".";

export default {
  title: "atoms/FontAwesome"
};

export const Brand = () => <FontAwesome prefix="brands" icon="twitter" />;
export const Solid = () => <FontAwesome prefix="solid" icon="rabbit" />;
export const Regular = () => <FontAwesome prefix="regular" icon="rabbit" />;
export const Fixed = () => (
  <>
    <FontAwesome prefix="brands" icon="twitter" fixed />
    <FontAwesome prefix="solid" icon="rabbit" fixed />
    <FontAwesome prefix="regular" icon="rabbit" fixed />
  </>
);
