import { Button, Container, createStyles, Dialog, DialogActions, DialogContent, DialogContentText, DialogTitle, makeStyles, TextField, Theme, Typography, WithStyles, withStyles } from "@material-ui/core";
import React, { Component } from "react";
import { RouteComponentProps, RouteProps, RouterProps } from "react-router";
import { ChallengeInfo, getChallengeInfo, solveChallenge } from "../api/Api";
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
    input: {
        display: 'block',
        margin: '10px'
    },
});


type ChallengeState = {
    challengeKey: string,
    groupKey: string,
    challengeInfo?: ChallengeInfo,
    dialogOpen: boolean,
    dialogContent?: string,
    textFieldValue: string,
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
            dialogOpen: false,
            dialogContent: undefined,
            textFieldValue: "",
        };
    }

    componentDidMount() {
        getChallengeInfo(this.state.groupKey, this.state.challengeKey).then(data => {
            this.setState({ challengeInfo: data });
        })
    }



    render() {
        const solveProblem = () => {
            solveChallenge(this.state.groupKey, this.state.challengeKey, this.state.textFieldValue).then(data => {
                this.setState({ dialogOpen: true, dialogContent: data });
            });
        }

        const { classes } = this.props;
        return <Container maxWidth="lg">
            <Typography variant="h4" className={classes.title}>{this.state.challengeInfo?.title}</Typography>
            <Typography variant="h6" className={classes.description}>
                {this.state.challengeInfo?.description.split("\n").map((i, key) => {
                    return <p className={classes.descriptionLine} key={key}>{i}</p>;
                })}
            </Typography>
            <TextField
                className={classes.input}
                label="input"
                multiline
                fullWidth
                rows={20}
                variant="outlined"
                value={this.state.textFieldValue}
                onChange={(e) => {
                    this.setState({ textFieldValue: e.target.value });
                }}
            />

            <Button className={classes.input} onClick={solveProblem} variant="contained" color="primary">
                Solve
            </Button>
            <Dialog
                open={this.state.dialogOpen}
                onClose={() => this.setState({ dialogOpen: false })}
                aria-labelledby="alert-dialog-title"
                aria-describedby="alert-dialog-description"
            >
                <DialogTitle id="alert-dialog-title">{"Answer"}</DialogTitle>
                <DialogContent>
                    <DialogContentText id="alert-dialog-description">
                        {this.state.dialogContent?.split('\n').map(str => <div>{str}</div>)}
                    </DialogContentText>
                </DialogContent>
                <DialogActions>
                    <Button onClick={() => this.setState({ dialogOpen: false })} color="primary" autoFocus>
                        OK
                    </Button>
                </DialogActions>
            </Dialog>
        </Container >;
    }
}

export default withStyles(styles)(Group)