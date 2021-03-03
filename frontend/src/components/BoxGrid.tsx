import React from 'react';
import { Link } from 'react-router-dom';

import Grid from '@material-ui/core/Grid';
import Paper from '@material-ui/core/Paper';
import { createStyles, makeStyles, Theme } from '@material-ui/core/styles';
import Typography from '@material-ui/core/Typography';

const useStyles = makeStyles((theme: Theme) =>
    createStyles({
        root: {
            flexGrow: 1,
            margin: theme.spacing(2)
        },
        link: {
            "text-decoration": "none",
        },
        paper: {
            height: 140,
            width: 200,
            "padding-top": 30,
            "text-align": "center",
            cursor: "pointer",
        },
        control: {
            padding: theme.spacing(2),
        },
    }),
);

export type GridItem = {
    key: string,
    displayName: string,
}

type GridProps = {
    baseUrl: string,
    items: GridItem[],
}

export function BoxGrid(props: GridProps) {
    const classes = useStyles();

    return (
        <Grid container className={classes.root} spacing={2}>
            <Grid item xs={12}>
                <Grid container justify="center" spacing={2}>
                    {props.items.map((value) => {
                        let link = `/${props.baseUrl}/${value.key}`;
                        return (
                            <Grid key={value.key} item>
                                <Link to={link} className={classes.link} >
                                    <Paper className={classes.paper}>
                                        <Typography variant="h5">{value.displayName}</Typography>
                                    </Paper>
                                </Link>
                            </Grid>
                        )
                    })}
                </Grid>
            </Grid>
        </Grid>
    );
}