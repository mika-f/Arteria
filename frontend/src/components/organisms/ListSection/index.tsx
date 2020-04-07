import React from "react";

import { ListContainer, ListItem } from "../../atoms/List";
import Link from "../../molecules/Link";
import Section from "../../molecules/Section";

type Props = {
  title: string;
  items: { href: string; text: string }[];
};

const ListSection: React.FC<Props> = ({ title, items }) => {
  return (
    <Section size="small" title={title}>
      <ListContainer>
        {items.map(w => (
          <ListItem key={w.href}>
            <Link href={w.href}>{w.text}</Link>
          </ListItem>
        ))}
      </ListContainer>
    </Section>
  );
};

export default ListSection;
