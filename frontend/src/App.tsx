import React from 'react';
import { BrowserRouter as Router, Route, Switch, Link } from 'react-router-dom';

import AppBar from '@material-ui/core/AppBar';
import CssBaseline from '@material-ui/core/CssBaseline';
import { createStyles, makeStyles, Theme } from '@material-ui/core/styles';
import Toolbar from '@material-ui/core/Toolbar';
import Typography from '@material-ui/core/Typography';
import { Breadcrumbs } from '@material-ui/core';
import HomeIcon from '@material-ui/icons/Home';

import Groups from './views/Groups';
import Group from './views/Group';
import Challenge from './views/Challenge';
import { ClassNameMap } from '@material-ui/styles';


const useStyles = makeStyles((theme: Theme) =>
  createStyles({
    root: {
      flexGrow: 1,
    },
    menuButton: {
      marginRight: theme.spacing(2),
    },

    // Breadcrumbs
    crumb: {
      color: '#fff',
      display: 'flex',
    },
    crumbLink: {
      textDecoration: 'none',
    },
    icon: {
      marginRight: theme.spacing(0.5),
      width: 20,
      height: 20,
    },
  }),
);


function renderRoute(classes: ClassNameMap, content: JSX.Element) {
  return <div className={classes.root}>
    <CssBaseline />
    <AppBar position="static">
      <Toolbar>
        <Breadcrumbs>
          <Link to="/" className={classes.crumbLink}>
            <Typography variant="h6" color="textPrimary" className={classes.crumb}>
              <HomeIcon className={classes.icon} />
            Groups
        </Typography>
          </Link>
          <Link to="/getting-started/installation/" className={classes.crumbLink}>
            <Typography variant="h6" color="textPrimary" className={classes.crumb}>
              Challenge
      </Typography>
          </Link>
          <Typography variant="h6" className={classes.crumb}>
            Breadcrumb
      </Typography>
        </Breadcrumbs>
      </Toolbar>
    </AppBar>
    {content}
  </div>
}


export default function App() {
  const classes = useStyles();
  const toolbar = <Toolbar></Toolbar>

  return (
    <Router>
      <Switch>
        <Route path="/groups/:groupKey/:challengeKey" render={
          props => {
            return renderRoute(classes, <Challenge {...props}></Challenge>);
          }
        } />
        <Route path="/groups/:groupKey" render={
          props => {
            return renderRoute(classes, <Group {...props}></Group>);
          }
        } />
        <Route path="/" render={
          props => {
            return renderRoute(classes, <Groups {...props}></Groups>);
          }
        } />
      </Switch>
    </Router >
  );
}
