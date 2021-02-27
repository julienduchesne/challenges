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

type GridProps = {
    items: string[],
}

export default function SpacingGrid(props: GridProps) {
    const classes = useStyles();

    return (
        <Grid container className={classes.root} spacing={2}>
            <Grid item xs={12}>
                <Grid container justify="center" spacing={2}>
                    {props.items.map((value) => (
                        <Grid key={value} item>
                            <Link to={`/${value}`} className={classes.link} >
                                <Paper className={classes.paper}>
                                    <Typography variant="h5">{value}</Typography>
                                </Paper>
                            </Link>
                        </Grid>
                    ))}
                </Grid>
            </Grid>
        </Grid>
    );
}