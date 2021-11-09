import { NgModule } from '@angular/core';
import { RouterModule, Routes } from '@angular/router';
import { GameComponent } from './components/game/game.component';
import { AppMainComponent } from './template-components/app-main/app.main.component';

const routes: Routes = [
  {
    path: '',
    redirectTo: '/Game',
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
    ],
  },
];

@NgModule({
  imports: [RouterModule.forRoot(routes)],
  exports: [RouterModule],
})
export class AppRoutingModule {}
