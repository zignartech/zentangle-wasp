import { NgModule } from '@angular/core';
import { RouterModule, Routes } from '@angular/router';
import {IcentivizedAITrainingComponent} from "./components/icentivized-aitraining/icentivized-aitraining.component";
import {AppMainComponent} from "./template-components/app-main/app.main.component";

const routes: Routes = [{
  path: '',
  component: AppMainComponent,
  children: [
    {
      path: 'Icentivized-AI-Training',
      component: IcentivizedAITrainingComponent
    }
  ]
}];

@NgModule({
  imports: [RouterModule.forRoot(routes)],
  exports: [RouterModule]
})
export class AppRoutingModule { }
