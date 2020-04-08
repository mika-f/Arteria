import React, { useEffect } from "react";
import GoogleAnalytics, { FieldsObject } from "react-ga";
import { RouteComponentProps } from "react-router-dom";

if (process.env.ARTERIA_GOOGLE_ANALYTICS) {
  GoogleAnalytics.initialize(process.env.ARTERIA_GOOGLE_ANALYTICS as string);
}

const withTracker = <P extends RouteComponentProps>(WrappedComponent: React.ComponentType<P>, options: FieldsObject = {}) => {
  const track = (page: string) => {
    GoogleAnalytics.set({ page, ...options });
    GoogleAnalytics.pageview(page);
  };

  const Hoc: React.FC<P> = props => {
    const { location } = props;

    useEffect(() => {
      window.setTimeout(() => {
        track(location.pathname);
      }, 0);
    }, [location.pathname]);

    // eslint-disable-next-line react/jsx-props-no-spreading
    return <WrappedComponent {...props} />;
  };

  return Hoc;
};

export default withTracker;
