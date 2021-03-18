import { Component } from "react";
import { RouteComponentProps, RouteProps, RouterProps } from "react-router";
import { getGroupInfo } from "../api/Api";
import { GridItem, BoxGrid } from "../components/BoxGrid";

type GroupState = {
    groupKey: string,
    displayName: string
    challenges: GridItem[]
}

export default class Group extends Component<RouteComponentProps, GroupState> {
    constructor(props: RouteComponentProps) {
        super(props);

        const { match: { params } } = props;
        const { groupKey } = params as {
            groupKey: string
        };

        this.state = {
            groupKey: groupKey,
            displayName: "",
            challenges: [],
        };
    }

    componentDidMount() {
        getGroupInfo(this.state.groupKey).then(data => {
            let challenges = data.challenges.map(i => ({ key: i.key, displayName: i.display_name }))
            this.setState({ displayName: data.name, challenges: challenges })
        })
    }

    render() {
        return <BoxGrid baseUrl={`groups/${this.state.groupKey}`} items={this.state.challenges} />;
    }
}