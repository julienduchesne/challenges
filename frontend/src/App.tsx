import React from 'react';
import { BrowserRouter as Router, Link, Route, Switch, useParams } from 'react-router-dom';

import AppBar from '@material-ui/core/AppBar';
import CssBaseline from '@material-ui/core/CssBaseline';
import { createStyles, makeStyles, Theme } from '@material-ui/core/styles';
import Toolbar from '@material-ui/core/Toolbar';
import Typography from '@material-ui/core/Typography';

import Groups from './views/Groups';
import Group from './views/Group';


const useStyles = makeStyles((theme: Theme) =>
  createStyles({
    root: {
      flexGrow: 1,
    },
    menuButton: {
      marginRight: theme.spacing(2),
    },
    title: {
      flexGrow: 1,
    },
  }),
);



export default function App() {
  const classes = useStyles();
  return (
    <Router>
      <div className={classes.root}>
        <CssBaseline />
        <AppBar position="static">
          <Toolbar>
            <Typography variant="h6" className={classes.title}>
              Challenges
          </Typography>
          </Toolbar>
        </AppBar>

        <Switch>
          <Route path="/groups/:groupKey" component={Group} />
          <Route path="/" component={Groups} />
        </Switch>
      </div>
    </Router >
  );
}
