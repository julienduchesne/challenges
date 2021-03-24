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


function renderRoute(classes: ClassNameMap, content: JSX.Element, groupId?: string, challengeId?: string) {
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
          {(groupId != undefined) &&
            <Link to={`/groups/${groupId}`} className={classes.crumbLink}>
              <Typography variant="h6" color="textPrimary" className={classes.crumb}>
                {groupId}
              </Typography>
            </Link>
          }
          {(challengeId != undefined) &&
            <Link to={`/groups/${groupId}/${challengeId}`} className={classes.crumbLink}>
              <Typography variant="h6" className={classes.crumb}>
                {challengeId}
              </Typography>
            </Link>
          }
        </Breadcrumbs>
      </Toolbar>
    </AppBar>
    {content}
  </div>
}


export default function App() {
  const classes = useStyles();
  return (
    <Router>
      <Switch>
        <Route path="/groups/:groupKey/:challengeKey" render={
          props => {
            return renderRoute(classes, <Challenge {...props}></Challenge>, props.match.params.groupKey, props.match.params.challengeKey);
          }
        } />
        <Route path="/groups/:groupKey" render={
          props => {
            return renderRoute(classes, <Group {...props}></Group>, props.match.params.groupKey);
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
