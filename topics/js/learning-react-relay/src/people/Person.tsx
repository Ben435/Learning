import { graphql, useFragment } from "react-relay"
import { PersonDetailsFragment$key } from "./__generated__/PersonDetailsFragment.graphql"

const personDetailsFragment = graphql`
    fragment PersonDetailsFragment on Person {
        name
        species {
            name
        }
    }
`

interface PersonProps {
  person: PersonDetailsFragment$key
}

export function Person({ person }: PersonProps) {
  const data = useFragment(personDetailsFragment, person)
  return <p>Person {data.name} of species {data.species?.name || 'unknown'}</p>
}
