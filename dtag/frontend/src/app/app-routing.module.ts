import { NgModule } from '@angular/core';
import { RouterModule, Routes } from '@angular/router';
import { GameComponent } from './components/game/game.component';
import { HomeComponent } from './components/home/home.component';
import { RoundSetupComponent } from './components/round-setup/round-setup.component';
import { StartPlayerComponent } from './components/start-player/start-player.component';
import { StartResearcherComponent } from './components/start-researcher/start-researcher.component';
import { AppMainComponent } from './template-components/app-main/app.main.component';

const routes: Routes = [
  {
    path: '',
    redirectTo: '/Home',
    pathMatch: 'full',
  },
  {
    path: '',
    component: AppMainComponent,
    children: [
      {
        path: 'Game',
        component: GameComponent,
      },
      {
        path: 'Home',
        component: HomeComponent,
      },
      {
        path: 'Start-Player',
        component: StartPlayerComponent,
      },
      {
        path: 'Start-Researcher',
        component: StartResearcherComponent,
      },
      {
        path: 'Setup',
        component: RoundSetupComponent,
      },
    ],
  },
];

@NgModule({
  imports: [RouterModule.forRoot(routes, { useHash: true })],
  exports: [RouterModule],
})
export class AppRoutingModule {}
