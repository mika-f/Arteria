import React from "react";
import Helmet from "react-helmet";

import PrivacyTemplate from "../../templates/Privacy";

const Privacy: React.FC = () => {
  const hasGoogleAnalytics = !!process.env.ARTERIA_GOOGLE_ANALYTICS;

  return (
    <>
      <Helmet>
        <title>プライバシーポリシー</title>
      </Helmet>
      <PrivacyTemplate hasGoogleAnalyticsTag={hasGoogleAnalytics} />
    </>
  );
};

export default Privacy;
