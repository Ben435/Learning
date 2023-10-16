import { PreloadedQuery, usePreloadedQuery } from "react-relay"
import { appMainQuery } from "../__generated__/appMainQuery.graphql"
import { appQuery } from "../app"
import { Person } from "./Person"

interface PeopleProps {
  queryReference: PreloadedQuery<appMainQuery>
}

export function People({ queryReference }: PeopleProps) {
  const data = usePreloadedQuery<appMainQuery>(appQuery, queryReference)

  return (
    <>
      <div>{data!.allPeople.edges.map(person => <Person key={person!.node.id} person={person!.node} />)}</div>
    </>
  )
}