import { Container, createStyles, makeStyles, TextField, Theme, Typography, WithStyles, withStyles } from "@material-ui/core";
import React, { Component } from "react";
import { RouteComponentProps, RouteProps, RouterProps } from "react-router";
import { ChallengeInfo, getChallengeInfo } from "../api/Api";
import { GridItem, BoxGrid } from "../components/BoxGrid";

const styles = (theme: Theme) => createStyles({
    title: {
        textAlign: 'center',
        margin: '20px',
    },
    description: {
        textAlign: "center",
    },
    descriptionLine: {
        margin: "0px"
    },
});

type ChallengeState = {
    challengeKey: string,
    groupKey: string,
    challengeInfo?: ChallengeInfo
}

class Group extends Component<RouteComponentProps & WithStyles<typeof styles>, ChallengeState> {
    constructor(props: RouteComponentProps & WithStyles<typeof styles>) {
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
        const { classes } = this.props;
        return <Container maxWidth="lg">
            <Typography variant="h4" className={classes.title}>{this.state.challengeInfo?.title}</Typography>
            <Typography variant="h6" className={classes.description}>
                {this.state.challengeInfo?.description.split("\n").map((i, key) => {
                    return <p className={classes.descriptionLine} key={key}>{i}</p>;
                })}
            </Typography>
            {this.state.challengeInfo?.variables.map((key) => {
                return <TextField
                    label={key}
                    multiline
                    rows={4}
                    defaultValue="Default Value"
                    variant="outlined"
                />
            })}
        </Container>;
    }
}

export default withStyles(styles)(Group)