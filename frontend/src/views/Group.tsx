import { Component } from "react";
import { getGroupInfo } from "../api/Api";
import { GridItem, BoxGrid } from "../components/BoxGrid";

type GroupState = {
    displayName: string
    challenges: GridItem[]
}

type GroupProps = {
    groupKey: string
}

export default class Group extends Component<GroupProps, GroupState> {
    constructor(props: GroupProps) {
        super(props);

        this.state = {
            displayName: "",
            challenges: [],
        };
    }

    componentDidMount() {
        getGroupInfo(this.props.groupKey).then(data => {
            let challenges = data.challenges.map(i => ({ key: i.key, displayName: i.display_name }))
            this.setState({ displayName: data.name, challenges: challenges })
        })
    }

    render() {
        return <BoxGrid baseUrl={`groups/${this.props.groupKey}`} items={this.state.challenges} />;
    }
}