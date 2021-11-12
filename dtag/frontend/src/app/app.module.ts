import { ClipboardModule } from '@angular/cdk/clipboard';
import { CommonModule } from '@angular/common';
import { HttpClientModule } from '@angular/common/http';
import { NgModule } from '@angular/core';
import { BrowserModule } from '@angular/platform-browser';
import { BrowserAnimationsModule } from '@angular/platform-browser/animations';
import { RouterModule } from '@angular/router';
import { RippleModule } from 'primeng/ripple';
import { SwiperModule } from 'swiper/angular';

import { AppRoutingModule } from './app-routing.module';
import { AppComponent } from './app.component';
import { GameComponent } from './components/game/game.component';
import { PrimengComponentsModule } from './template-components/primeng-components.module';
import { TopbarComponent } from './template-components/topbar/topbar.component';
import { HomeComponent } from './components/home/home.component';
import { StartPlayerComponent } from './components/start-player/start-player.component';
import { ActivePlayersComponent } from './common/active-players/active-players.component';
import { StartResearcherComponent } from './components/start-researcher/start-researcher.component';

@NgModule({
  declarations: [
    AppComponent,
    GameComponent,
    HomeComponent,
    StartPlayerComponent,
    ActivePlayersComponent,
    StartResearcherComponent,
  ],
  imports: [
    AppRoutingModule,
    BrowserAnimationsModule,
    BrowserModule,
    HttpClientModule,
    RouterModule,
    PrimengComponentsModule,
    ClipboardModule,
    RippleModule,
    SwiperModule,
  ],
  providers: [],
  bootstrap: [AppComponent],
})
export class AppModule {}
