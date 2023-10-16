import { PreloadedQuery, usePreloadedQuery } from "react-relay"
import { Suspense } from "react"
import { appMainQuery } from "../__generated__/appMainQuery.graphql"
import { appQuery } from "../app"
import { Person } from "./Person"

interface PeopleProps {
    queryReference: PreloadedQuery<appMainQuery>
}

export function People({ queryReference }: PeopleProps) {
    const data = usePreloadedQuery<appMainQuery>(appQuery, queryReference)

    return (
        <Suspense fallback={<p>loading people</p>}>
            <div>{data!.allPeople.edges.map(person => <Person key={person!.node.id} person={person!.node}/>)}</div>
        </Suspense>
    )
}