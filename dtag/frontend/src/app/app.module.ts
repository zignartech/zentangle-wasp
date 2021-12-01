import { ClipboardModule } from '@angular/cdk/clipboard';
import { CommonModule } from '@angular/common';
import { HttpClientModule } from '@angular/common/http';
import { NgModule } from '@angular/core';
import { FormsModule, ReactiveFormsModule } from '@angular/forms';
import { BrowserModule } from '@angular/platform-browser';
import { BrowserAnimationsModule } from '@angular/platform-browser/animations';
import { RouterModule } from '@angular/router';
import { NgxDropzoneModule } from 'ngx-dropzone';
import { RippleModule } from 'primeng/ripple';
import { SwiperModule } from 'swiper/angular';

import { AppRoutingModule } from './app-routing.module';
import { AppComponent } from './app.component';
import { GameComponent } from './components/game/game.component';
import { PrimengComponentsModule } from './template-components/primeng-components.module';
import { HomeComponent } from './components/home/home.component';
import { StartPlayerComponent } from './components/start-player/start-player.component';
import { ActivePlayersComponent } from './common/active-players/active-players.component';
import { StartResearcherComponent } from './components/start-researcher/start-researcher.component';
import { RoundSetupComponent } from './components/round-setup/round-setup.component';
import { DIDComponent } from './common/did/did.component';
import { BalanceComponent } from './common/balance/balance.component';

@NgModule({
  declarations: [
    AppComponent,
    GameComponent,
    HomeComponent,
    StartPlayerComponent,
    ActivePlayersComponent,
    StartResearcherComponent,
    RoundSetupComponent,
    DIDComponent,
    BalanceComponent,
  ],
  imports: [
    AppRoutingModule,
    BrowserAnimationsModule,
    BrowserModule,
    CommonModule,
    ClipboardModule,
    FormsModule,
    HttpClientModule,
    NgxDropzoneModule,
    PrimengComponentsModule,
    RouterModule,
    ReactiveFormsModule,
    RippleModule,
    SwiperModule,
  ],
  providers: [],
  bootstrap: [AppComponent],
})
export class AppModule {}
