import React from 'react';
import { BrowserRouter as Router, Link, Route, Switch, useParams } from 'react-router-dom';

import AppBar from '@material-ui/core/AppBar';
import CssBaseline from '@material-ui/core/CssBaseline';
import { createStyles, makeStyles, Theme } from '@material-ui/core/styles';
import Toolbar from '@material-ui/core/Toolbar';
import Typography from '@material-ui/core/Typography';

import { SpacingGrid, GridItem } from './SpacingGrid';
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

          <Route path="/groups/:groupName">
            <Group />
          </Route>
          <Route path="/">
            <Groups />
          </Route>

        </Switch>
      </div>
    </Router>
  );
}

type GroupsState = {
  groups: GridItem[]
}

type GroupNameApiResult = {
  key: string
  display_name: string
}

class Groups extends Component<{}, GroupsState> {
  constructor(props: {}) {
    super(props);

    this.state = {
      groups: []
    };
  }

  componentDidMount() {
    fetch(`${API_URL}/groups`)
      .then(res => res.json())
      .then((data: GroupNameApiResult[]) => {
        let groups = data.map(i => ({ key: i.key, displayName: i.display_name }))
        this.setState({ groups: groups })
      }).catch(console.log);
  }

  render() {
    return <SpacingGrid baseUrl="groups" items={this.state.groups} />;
  }
}

interface GroupParams {
  groupName: string
}

function Group() {
  let { groupName } = useParams<GroupParams>();
  return <h2>{groupName}</h2>;
}
