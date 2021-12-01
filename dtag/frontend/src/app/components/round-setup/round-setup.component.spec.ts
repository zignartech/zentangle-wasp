import { ComponentFixture, TestBed } from '@angular/core/testing';

import { RoundSetupComponent } from './round-setup.component';

describe('RoundSetupComponent', () => {
  let component: RoundSetupComponent;
  let fixture: ComponentFixture<RoundSetupComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [ RoundSetupComponent ]
    })
    .compileComponents();
  });

  beforeEach(() => {
    fixture = TestBed.createComponent(RoundSetupComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
