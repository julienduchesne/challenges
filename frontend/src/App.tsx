import React from 'react';
import { BrowserRouter as Router, Link, Route, Switch, useParams } from 'react-router-dom';

import AppBar from '@material-ui/core/AppBar';
import CssBaseline from '@material-ui/core/CssBaseline';
import { createStyles, makeStyles, Theme } from '@material-ui/core/styles';
import Toolbar from '@material-ui/core/Toolbar';
import Typography from '@material-ui/core/Typography';

import SpacingGrid from './SpacingGrid';
import { Component } from 'react';

const API_URL = "http://localhost:8000/api"

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
          <Route path="/">
            <Groups />
          </Route>
          <Route path="/about">
            <About />
          </Route>
          <Route path="/users/:userId">
            <Users />
          </Route>

        </Switch>
      </div>
    </Router>
  );
}

type GroupsState = {
  groups: string[]
}

class Groups extends Component<{}, GroupsState> {
  constructor(props: {}) {
    super(props);

    this.state = {
      groups: []
    };
  }

  componentDidMount() {
    fetch(`${API_URL}/groups`).then(res => res.json()).then(data => { this.setState({ groups: data }) }).catch(console.log);

  }

  render() {
    return <SpacingGrid items={this.state.groups} />;
  }
}

function About() {
  return <h2>About</h2>;
}

interface ParamTypes {
  userId: string
}

function Users() {
  let { userId } = useParams<ParamTypes>();
  return <h2>Users: {userId}</h2>;
}