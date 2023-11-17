import React from 'react'
import { graphql, useFragment } from 'react-relay'
import { PlanetDetailsFragment$key } from './__generated__/PlanetDetailsFragment.graphql'

const getPlanetDetails = graphql`
    fragment PlanetDetailsFragment on Planet {
        name
        surfaceWater
    }
`

export interface PlanetProps {
  planet: PlanetDetailsFragment$key
}

export const Planet = ({planet}: PlanetProps) => {
  const planetDetails = useFragment(getPlanetDetails, planet)
  return (
    <div style={{color: surfaceWaterMetric(planetDetails.surfaceWater!)}}>{planetDetails.name}</div>
  )
}

const surfaceWaterMetric = (surfaceWater: number) => surfaceWater > 50 ? 'blue' : 'red'
