import { Component } from "react";
import { getGroupNames } from "../api/Api";
import { GridItem, BoxGrid } from "../components/BoxGrid";

type GroupsState = {
    groups: GridItem[]
}

export default class Groups extends Component<{}, GroupsState> {
    constructor(props: {}) {
        super(props);

        this.state = {
            groups: []
        };
    }

    componentDidMount() {
        getGroupNames().then(data => {
            let groups = data.map(i => ({ key: i.key, displayName: i.display_name }))
            this.setState({ groups: groups })
        })
    }

    render() {
        return <BoxGrid baseUrl="groups" items={this.state.groups} />;
    }
}