import { PreloadedQuery, usePreloadedQuery } from "react-relay"
import { Suspense } from "react"
import { AppQuery } from "../__generated__/AppQuery.graphql"
import { appQuery } from "../appQuery"
import { Person } from "./Person"

interface PeopleProps {
    queryReference: PreloadedQuery<AppQuery>
}

export function People({ queryReference }: PeopleProps) {
    const data = usePreloadedQuery<AppQuery>(appQuery, queryReference)

    return (
        <Suspense fallback={<p>loading people</p>}>
            <div>{data.allPeople!.people!.map(person => <Person person={person!}/>)}</div>
        </Suspense>
    )
}