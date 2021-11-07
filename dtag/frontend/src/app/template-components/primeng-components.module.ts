import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import {RouterModule} from "@angular/router";
import {InlineSVGModule} from "ng-inline-svg";
import { BreadcrumbModule } from 'primeng/breadcrumb';
import { ButtonModule } from 'primeng/button';
import { CalendarModule } from 'primeng/calendar';
import { CardModule } from 'primeng/card';
import { CheckboxModule } from 'primeng/checkbox';
import { ChipsModule } from 'primeng/chips';
import { ConfirmDialogModule } from 'primeng/confirmdialog';
import { ConfirmPopupModule } from 'primeng/confirmpopup';
import { DialogModule } from 'primeng/dialog';
import { DividerModule } from 'primeng/divider';
import { InputNumberModule } from 'primeng/inputnumber';
import { InputTextModule } from 'primeng/inputtext';
import { InputTextareaModule } from 'primeng/inputtextarea';
import { ListboxModule } from 'primeng/listbox';
import { MenuModule } from 'primeng/menu';
import { MessagesModule } from 'primeng/messages';
import { MultiSelectModule } from 'primeng/multiselect';
import { PaginatorModule } from 'primeng/paginator';
import { PasswordModule } from 'primeng/password';
import {RippleModule} from "primeng/ripple";
import { ScrollPanelModule } from 'primeng/scrollpanel';
import { SkeletonModule } from 'primeng/skeleton';
import { SidebarModule } from 'primeng/sidebar';
import { StepsModule } from 'primeng/steps';
import { TableModule } from 'primeng/table';
import {TagModule} from "primeng/tag";
import {TimelineModule} from "primeng/timeline";
import { ToastModule } from 'primeng/toast';
import { ProgressSpinnerModule } from 'primeng/progressspinner';
import { AppMainComponent } from './app-main/app.main.component';
import { MenuComponent } from './menu/menu.component';
import { MenuItemComponent } from './menu-item/menu-item.component';
import { TopbarComponent } from './topbar/topbar.component';

@NgModule({
  declarations: [
    AppMainComponent,
    MenuComponent,
    MenuItemComponent,
    TopbarComponent,
  ],
  imports: [CommonModule, RippleModule, RouterModule, CommonModule, InlineSVGModule],
  exports: [
    BreadcrumbModule,
    ButtonModule,
    CalendarModule,
    CardModule,
    CheckboxModule,
    ChipsModule,
    ConfirmDialogModule,
    ConfirmPopupModule,
    DialogModule,
    DividerModule,
    InputNumberModule,
    InputTextModule,
    InputTextareaModule,
    ListboxModule,
    MenuModule,
    MessagesModule,
    MultiSelectModule,
    PaginatorModule,
    PasswordModule,
    RouterModule,
    ScrollPanelModule,
    SkeletonModule,
    SidebarModule,
    StepsModule,
    TableModule,
    TagModule,
    TimelineModule,
    TopbarComponent,
    ToastModule,
    ProgressSpinnerModule,
  ],
})
export class PrimengComponentsModule {}
