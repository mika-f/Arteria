import React from "react";

import Alert from ".";

export default {
  title: "molecules/Alert"
};

export const Info = () => (
  <Alert title="Information" variant="info">
    Alert Information
  </Alert>
);

export const Warning = () => (
  <Alert title="Warning" variant="warning">
    Alert Warning
  </Alert>
);

export const Error = () => (
  <Alert title="Error" variant="error">
    Alert Error
  </Alert>
);
