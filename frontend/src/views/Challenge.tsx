import { Typography } from "@material-ui/core";
import React, { Component } from "react";
import { RouteComponentProps, RouteProps, RouterProps } from "react-router";
import { ChallengeInfo, getChallengeInfo } from "../api/Api";
import { GridItem, BoxGrid } from "../components/BoxGrid";

type ChallengeState = {
    challengeKey: string,
    groupKey: string,
    challengeInfo?: ChallengeInfo
}

export default class Group extends Component<RouteComponentProps, ChallengeState> {
    constructor(props: RouteComponentProps) {
        super(props);

        const { match: { params } } = props;
        const { challengeKey, groupKey } = params as {
            groupKey: string,
            challengeKey: string,
        };

        this.state = {
            groupKey: groupKey,
            challengeKey: challengeKey,
            challengeInfo: undefined,
        };
    }

    componentDidMount() {
        getChallengeInfo(this.state.groupKey, this.state.challengeKey).then(data => {
            this.setState({ challengeInfo: data });
        })
    }

    render() {
        return <div>
            <Typography variant="h6">{this.state.challengeInfo?.title}</Typography>
            <Typography variant="h6">{this.state.challengeInfo?.description}</Typography>
        </div>;
    }
}