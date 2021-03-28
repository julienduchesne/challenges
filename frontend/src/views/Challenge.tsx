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
    dialogContent?: string
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
        };
    }

    componentDidMount() {
        getChallengeInfo(this.state.groupKey, this.state.challengeKey).then(data => {
            this.setState({ challengeInfo: data });
        })
    }



    render() {
        const { classes } = this.props;

        let variables = this.state.challengeInfo?.variables || [];

        const textFields = Object.assign({}, ...variables.map((key) => (
            {
                [key]: <TextField
                    ref={React.createRef()}
                    className={classes.input}
                    label={key}
                    multiline
                    fullWidth
                    rows={12}
                    defaultValue=""
                    variant="outlined"
                />
            }
        )));

        const solveProblem = () => {
            this.setState({ dialogOpen: true, dialogContent: "test3" })
        }
        return <Container maxWidth="lg">
            <Typography variant="h4" className={classes.title}>{this.state.challengeInfo?.title}</Typography>
            <Typography variant="h6" className={classes.description}>
                {this.state.challengeInfo?.description.split("\n").map((i, key) => {
                    return <p className={classes.descriptionLine} key={key}>{i}</p>;
                })}
            </Typography>
            {Object.values(textFields)}

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
                        {this.state.dialogContent}
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