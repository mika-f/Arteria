import React from "react";
import { Helmet } from "react-helmet";
import { BrowserRouter, Route } from "react-router-dom";

import withTracker from "./components/hoc/withTracker";
import About from "./components/pages/About";
import Root from "./components/pages/Root";
import Terms from "./components/pages/Terms";

const App: React.FC = () => {
  return (
    <>
      <Helmet defaultTitle="Arteria" titleTemplate="%s | Arteria" />
      <BrowserRouter>
        <Route path="/" component={withTracker(Root)} exact />
        <Route path="/about" component={withTracker(About)} exact />
        <Route path="/terms" component={withTracker(Terms)} exact />
      </BrowserRouter>
    </>
  );
};

export default App;
